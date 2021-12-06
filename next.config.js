/** @type {import('next').NextConfig} */
module.exports = {
  reactStrictMode: true,
  async rewrites() {
    return [
      {
        source: "/triple-tac-toe",
        destination: `https://triple-tac-toe.vidhan.io/triple-tac-toe`,
      },
      {
        source: "/triple-tac-toe/:path*",
        destination: `https://triple-tac-toe.vidhan.io/triple-tac-toe/:path*`,
      },
      {
        source: "/resume",
        destination: `https://resume.vidhan.io/resume`,
      },
      {
        source: "/resume/:path*",
        destination: `https://resume.vidhan.io/resume/:path*`,
      },
    ];
  },
};
