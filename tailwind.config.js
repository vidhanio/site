module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    extend: {
      fontFamily: {
        sans: ["Inter", "sans-serif"],
        mono: ["'Berkeley Mono'", "monospace"],
      },
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
