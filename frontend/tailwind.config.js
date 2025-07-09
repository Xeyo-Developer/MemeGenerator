/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/**/*.{js,jsx,ts,tsx}",
    ],
    theme: {
        extend: {
            animation: {
                'spin': 'spin 1s linear infinite',
                'pulse': 'pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite',
            },
            backdropBlur: {
                xl: '24px',
            },
            boxShadow: {
                'inner': 'inset 0 2px 4px 0 rgba(0, 0, 0, 0.06)',
            },
            gradientColorStops: {
                'purple-600': '#9333ea',
                'blue-600': '#2563eb',
                'indigo-800': '#3730a3',
            }
        },
    },
    plugins: [],
}