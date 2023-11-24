module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    extend: {
      fontFamily: {
        mono: ["'Berkeley Mono'", "monospace"],
      },
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
