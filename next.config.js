/** @type {import('next').NextConfig} */
module.exports = {
  reactStrictMode: true,
  async rewrites() {
    return [
      {
        source: "/:path*",
        destination: `/`,
      },
      {
        source: "/triple-tac-toe/:path*",
        destination: `triple-tac-toe.vidhan.io`,
      },
      {
        source: "/resume/:path*",
        destination: `resume.vidhan.io`,
      },
    ];
  },
};
