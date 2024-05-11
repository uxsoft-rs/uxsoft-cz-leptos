/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./style/**/*.scss", "./src/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/typography'),
    require("daisyui")
  ],
  daisyui: {
    themes: [
      {
        uxsoft: {
          "primary": "darkOrange",
          "primary-content": "white",
          "primary-focus": "orange",
          "--rounded-box": "0px", // 6px border radius rounded-box utility class, used in card and other large boxes
          "--rounded-btn": "0px", // 3px border radius rounded-btn utility class, used in buttons and similar element
          "--rounded-badge": "1.9rem",
          "--btn-text-case": "none", // set default text transform for buttons

          ".btn": {
            "font-weight": "normal"
          },

          "secondary": "#f000b8",
          "accent": "#1dcdbc",
          "neutral": "#2b3440",
          "base-100": "#ffffff",
          "info": "#3abff8",
          "success": "#36d399",
          "warning": "#fbbd23",
          "error": "#f87272",
        },
      },
    ],
  }
}