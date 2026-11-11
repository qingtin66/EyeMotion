import { t, setLocale } from './i18n';

/** 创建语言选择菜单（桌面端保留语言能力，默认跟随系统语言）。 */
export function createLanguageMenu(): void {
    const menu = document.createElement('div');
    menu.id = 'language-menu';
    menu.className = 'language-menu';

    const locales = [
        { code: 'en', name: 'English' },
        { code: 'zh-Hans', name: '简体中文' },
        { code: 'zh-Hant', name: '繁體中文' }
    ];

    locales.forEach((locale) => {
        const option = document.createElement('div');
        option.className = 'language-option';
        option.textContent = locale.name;
        option.addEventListener('click', () => {
            setLocale(locale.code);
            menu.classList.remove('visible');
        });
        menu.appendChild(option);
    });

    const closeButton = document.createElement('button');
    closeButton.textContent = t('close') || 'Close';
    closeButton.addEventListener('click', () => {
        menu.classList.remove('visible');
    });

    const footer = document.createElement('div');
    footer.className = 'language-menu-footer';
    footer.appendChild(closeButton);

    menu.appendChild(footer);
    document.body.appendChild(menu);
}
