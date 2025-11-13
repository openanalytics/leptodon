/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["./node_modules/flowbite/**/*.js", "*.html", "./src/**/*.rs"],
  },
  darkMode: "class",
  plugins: [require("flowbite/plugin")],
  theme: {
    colors: {
      oablue: "#32a6d3",
      "oa-red": "#e52323",
      "oa-blue-darker": "#00729c",
      "oa-red-2": "#be1717",
      "oa-docs-blue": "#30638e",
    },
    extend: {
      aria: {
        currentPage: 'current="page"',
      },
    },
  },
};
