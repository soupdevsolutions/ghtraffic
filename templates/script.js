function setTheme(theme) {
    if (theme === 'dark') {
        document.getElementById('root').classList.add('dark');
        localStorage.setItem('theme', 'dark');
    } else {
        document.getElementById('root').classList.remove('dark');
        localStorage.setItem('theme', 'light');
    }
}

document.addEventListener('DOMContentLoaded', () => {
    const storedTheme = localStorage.getItem('theme');
    if (storedTheme) {
        setTheme(storedTheme);
    } else {
        setTheme('light'); // Default to light if no preference is saved
    }

    // Add event listener for the theme toggle button
    const themeToggleBtn = document.getElementById('theme-toggle');
    themeToggleBtn.addEventListener('click', () => {
        const isDarkMode = document.getElementById('root').classList.contains('dark');
        setTheme(isDarkMode ? 'light' : 'dark');
    });
});