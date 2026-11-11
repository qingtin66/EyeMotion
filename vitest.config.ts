import { defineConfig } from 'vitest/config';

// EyeMotion 前端单元测试配置（Phase1 测试基座）
// - jsdom 环境：i18n / audio 等纯逻辑可在无浏览器环境运行
// - 覆盖率仅统计被单测覆盖的纯逻辑模块，阈值见 coverage.thresholds
export default defineConfig({
  test: {
    environment: 'jsdom',
    include: ['ui/js/**/*.test.ts'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'text-summary'],
      include: ['ui/js/i18n.ts', 'ui/js/audio.ts'],
      thresholds: {
        lines: 60,
        statements: 60,
        functions: 60,
        branches: 50,
      },
    },
  },
});
