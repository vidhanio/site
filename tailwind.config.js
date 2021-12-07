module.exports = {
  purge: ["./pages/**/*.{js,ts,jsx,tsx}", "./components/**/*.{js,ts,jsx,tsx}"],
  darkMode: "media",
  theme: {
    extend: {
      fontSize: {
        none: ["0rem", "1.25rem"],
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
