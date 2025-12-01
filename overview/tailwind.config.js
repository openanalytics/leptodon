/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: [
      "./node_modules/flowbite/**/*.js",
      "*.html",
      "./src/**/*.rs",
      "../src",
      ".tailwind",
    ],
  },
  darkMode: "selector",
  plugins: [require("flowbite/plugin")],
  theme: {
    colors: {
      "oa-blue": "#32a6d3",
      "oa-red": "#e52323",
      "oa-blue-darker": "#00729c",
      "oa-red-2": "#be1717",
      "oa-docs-blue": "#30638e",
      "oa-gray": "#e6e6e6",
      "oa-gray-darker": "#c3c3c3", // 15% darker
    },
    extend: {
      aria: {
        currentPage: 'current="page"',
      },
    },
  },
};
