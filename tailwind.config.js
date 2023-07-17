/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },

  theme: {
    extend: {
      screens: {
        "4xl": "1920px",
      },
    },
    colors: {
      transparent: "transparent",
      red: "#EF3939",
      pink: "#F0ADA8",
      eggshell: "#F1FAEE",
      white: "#F2F8FA",
      light_blue: "#A8DADC",
      beige: "#D2D7B4",
      dark_blue: "#324571",
      purple: "#181139",
      black: "#0F191C",
    },
  },
}