module.exports = {
  purge: ["./pages/**/*.{js,ts,jsx,tsx}", "./components/**/*.{js,ts,jsx,tsx}"],
  darkMode: "media",
  theme: {
    extend: {
      minHeight: {
        "1/2-screen": "75vh",
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
