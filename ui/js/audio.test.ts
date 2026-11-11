import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { AudioPlayer } from './audio';

// 轻量 Web Audio 模拟：只实现 audio.ts 实际用到的节点/参数接口
class MockParam {
    value = 0;
    setValueAtTime(_v: number, _t: number) {
        return this;
    }
    linearRampToValueAtTime(_v: number, _t: number) {
        return this;
    }
    exponentialRampToValueAtTime(_v: number, _t: number) {
        return this;
    }
    cancelScheduledValues(_t: number) {
        return this;
    }
}

class MockNode {
    connect(_dest?: unknown) {
        return this;
    }
    disconnect() {}
    start(_t?: number) {}
    stop(_t?: number) {}
}

class MockGain extends MockNode {
    gain = new MockParam();
}
class MockOscillator extends MockNode {
    type = 'sine';
    frequency = new MockParam();
}
class MockFilter extends MockNode {
    type = 'lowpass';
    frequency = new MockParam();
}
class MockBufferSource extends MockNode {
    buffer: MockBuffer | null = null;
}

class MockBuffer {
    private readonly len: number;
    constructor(len = 128) {
        this.len = len;
    }
    getChannelData(_channel: number) {
        return new Float32Array(this.len);
    }
}

class MockAudioContext {
    state: AudioContextState = 'running';
    currentTime = 0;
    sampleRate = 48000;
    destination = new MockNode();
    createGain() {
        return new MockGain();
    }
    createOscillator() {
        return new MockOscillator();
    }
    createBiquadFilter() {
        return new MockFilter();
    }
    createBufferSource() {
        return new MockBufferSource();
    }
    createBuffer(_channels: number, length: number, _sampleRate: number) {
        return new MockBuffer(length);
    }
    resume() {
        this.state = 'running';
        return Promise.resolve();
    }
}

function setAudioContext(value: unknown) {
    (window as unknown as { AudioContext?: unknown; webkitAudioContext?: unknown }).AudioContext = value;
    (window as unknown as { webkitAudioContext?: unknown }).webkitAudioContext = undefined;
}

describe('AudioPlayer 默认与安全空操作', () => {
    afterEach(() => {
        vi.useRealTimers();
        vi.unstubAllGlobals();
        vi.restoreAllMocks();
        setAudioContext(undefined);
    });

    it('构造默认开启 bgm/sfx，未 init 时调用不报错', () => {
        const p = new AudioPlayer();
        expect(p.bgmEnabled).toBe(true);
        expect(p.sfxEnabled).toBe(true);
        expect(() => {
            p.startBGM();
            p.stopBGM();
            p.playBounce();
            p.playClick();
            p.setVolume(0.5);
        }).not.toThrow();
    });

    it('无 AudioContext 时 init 走兜底分支且可重复调用', () => {
        setAudioContext(undefined);
        const p = new AudioPlayer();
        expect(() => p.init()).not.toThrow();
        expect(() => p.init()).not.toThrow();
    });
});

describe('AudioPlayer 音频合成路径（MockAudioContext）', () => {
    beforeEach(() => {
        setAudioContext(MockAudioContext);
    });
    afterEach(() => {
        vi.useRealTimers();
        vi.restoreAllMocks();
        setAudioContext(undefined);
    });

    it('init 成功后可播放 bounce/click 并调音量', () => {
        const p = new AudioPlayer();
        p.init();
        expect(() => {
            p.resume();
            p.playBounce();
            p.playClick();
            p.setVolume(0.3);
        }).not.toThrow();
    });

    it('sfx 关闭时 playBounce/playClick 直接返回', () => {
        const p = new AudioPlayer();
        p.init();
        p.sfxEnabled = false;
        expect(() => {
            p.playBounce();
            p.playClick();
        }).not.toThrow();
    });

    it('resume 在 suspended 状态会调用 ctx.resume()', async () => {
        const p = new AudioPlayer();
        p.init();
        const ctx = (p as unknown as { audioCtx: MockAudioContext }).audioCtx;
        ctx.state = 'suspended';
        await p.resume();
        expect(ctx.state).toBe('running');
    });

    it('BGM 循环可运行、切换与停止', () => {
        vi.useFakeTimers();
        const p = new AudioPlayer();
        p.init();
        p.startBGM(); // 触发 BGMPlayer.start -> playLoop
        vi.advanceTimersByTime(20000); // 跑多拍，覆盖 kick/hihat/乐器/颤音分支
        p.toggleMusic(); // 关闭
        p.toggleMusic(); // 开启
        p.stopBGM();
        expect(p.bgmEnabled).toBe(true);
        expect(() => p.startBGM()).not.toThrow();
    });
});
