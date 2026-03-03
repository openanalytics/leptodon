/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: [
      "./node_modules/flowbite/**/*.js",
      "*.html",
      "./src/**/*.rs",
      "../src/**/*.rs",
      ".tailwind",
    ],
  },
  darkMode: "selector",
  plugins: [require("flowbite/plugin")],
  theme: {
    colors: {
      "oa-blue-lighter": "#5bb8dc",
      "oa-blue": "#32a6d3",
      "oa-soft": "#32a6d310",
      "oa-blue-darker": "#00729c",
      "oa-red": "#e52323",
      "oa-red-darker": "#be1717",
      "oa-gray": "#e6e6e6",
      "oa-gray-mid": "#d6d6d6",
      "oa-gray-darker": "#c3c3c3", // 15% darker than oa-gray
      "oa-docs-blue": "#30638e",
      "codeblock-light": "#fafdff",
      "codeblock-dark": "#04121B",
      success: "var(--success)",
      "success-soft": "var(--success-soft)",
      warning: "var(--warning)",
      "warning-soft": "var(--warning-soft)",
      danger: "var(--danger)",
      "danger-soft": "var(--danger-soft)",
    },
    extend: {
      aria: {
        "current-page": 'current="page"',
      },
    },
  },
};
