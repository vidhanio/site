module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    extend: {
      fontFamily: {
        mono: ["'Cartograph CF'", "monospace"],
      },
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
