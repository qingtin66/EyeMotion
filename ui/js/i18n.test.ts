import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

// 屏蔽真实 Tauri 桥，setLocale 时只验证调用参数
vi.mock('./bridge', () => ({
    Bridge: { invoke: vi.fn().mockResolvedValue(undefined) }
}));

import { Bridge } from './bridge';

const sample = {
    app: { title: 'EyeMotion', start: '开始' },
    greeting: '你好 {name}',
    plain: 'plain text',
    nested: { level2: { level3: 'deep' } }
};

function stubFetchResolve(data: unknown) {
    const mock = vi.fn().mockResolvedValue({ json: () => Promise.resolve(data) });
    vi.stubGlobal('fetch', mock);
    return mock;
}

function stubFetchReject() {
    const mock = vi.fn().mockRejectedValue(new Error('network down'));
    vi.stubGlobal('fetch', mock);
    return mock;
}

/** 让 loadTranslations 的 Promise 链（fetch -> json -> then/catch）跑完。 */
function flush(): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, 0));
}

type I18nModule = typeof import('./i18n');

async function makeI18n(): Promise<{ inst: InstanceType<I18nModule['I18n']>; mod: I18nModule }> {
    const mod = await import('./i18n');
    const inst = new mod.I18n();
    await flush();
    return { inst, mod };
}

describe('i18n.t 纯逻辑', () => {
    beforeEach(() => {
        vi.stubGlobal('navigator', { language: 'en-US' });
        stubFetchResolve(sample);
    });
    afterEach(() => {
        vi.unstubAllGlobals();
        vi.clearAllMocks();
    });

    it('默认 locale 为 en', async () => {
        const { inst } = await makeI18n();
        expect(inst.getLocale()).toBe('en');
    });

    it('按点分路径解析嵌套键', async () => {
        const { inst } = await makeI18n();
        expect(inst.t('nested.level2.level3')).toBe('deep');
        expect(inst.t('app.title')).toBe('EyeMotion');
    });

    it('缺失键时原样返回 key', async () => {
        const { inst } = await makeI18n();
        expect(inst.t('nope.missing')).toBe('nope.missing');
        expect(inst.t('app.nothere')).toBe('app.nothere');
    });

    it('支持 {param} 占位符替换，缺参替换为空串', async () => {
        const { inst } = await makeI18n();
        expect(inst.t('greeting', { name: 'Tom' })).toBe('你好 Tom');
        expect(inst.t('greeting')).toBe('你好 ');
    });

    it('值为对象时返回 key', async () => {
        const { inst } = await makeI18n();
        expect(inst.t('app')).toBe('app');
    });

    it('导出的小写 t 助手可用', async () => {
        const mod = await import('./i18n');
        await flush();
        expect(mod.t('plain')).toBe('plain text');
    });
});

describe('i18n locale 探测', () => {
    afterEach(() => {
        vi.unstubAllGlobals();
        vi.clearAllMocks();
    });

    const cases: Array<[string, string]> = [
        ['en-US', 'en'],
        ['fr-FR', 'en'],
        ['zh-CN', 'zh-Hans'],
        ['zh-Hans-CN', 'zh-Hans'],
        ['zh-TW', 'zh-Hant'],
        ['zh-HK', 'zh-Hant']
    ];

    for (const [navLang, expected] of cases) {
        it(`navigator.language=${navLang} -> ${expected}`, async () => {
            vi.stubGlobal('navigator', { language: navLang });
            stubFetchResolve(sample);
            const { inst } = await makeI18n();
            expect(inst.getLocale()).toBe(expected);
        });
    }

    it('加载失败时回退 en，不抛异常', async () => {
        vi.stubGlobal('navigator', { language: 'zh-CN' });
        stubFetchReject();
        const mod = await import('./i18n');
        const inst = new mod.I18n();
        await flush();
        // 非 en 失败后回退请求 en，仍失败则清空，locale 保留探测值
        expect(inst.getLocale()).toBe('zh-Hans');
        expect(inst.t('any.key')).toBe('any.key');
    });
});

describe('i18n setLocale', () => {
    beforeEach(() => {
        vi.stubGlobal('navigator', { language: 'en-US' });
        stubFetchResolve(sample);
    });
    afterEach(() => {
        vi.unstubAllGlobals();
        vi.clearAllMocks();
    });

    it('切换语言并调用 Bridge.invoke(set_language)', async () => {
        const { inst } = await makeI18n();
        await inst.setLocale('zh-Hant');
        expect(inst.getLocale()).toBe('zh-Hant');
        expect(Bridge.invoke).toHaveBeenCalledWith('set_language', { language: 'zh-Hant' });
    });

    it('Bridge 失败时仍更新本地 locale 且不抛异常', async () => {
        const { inst } = await makeI18n();
        (Bridge.invoke as unknown as { mockRejectedValueOnce: (e: Error) => void }).mockRejectedValueOnce(
            new Error('no tauri')
        );
        await expect(inst.setLocale('zh-Hans')).resolves.toBeUndefined();
        expect(inst.getLocale()).toBe('zh-Hans');
    });
});
