/** @type {import('next').NextConfig} */
module.exports = {
  reactStrictMode: true,
  async rewrites() {
    return [
      {
        source: "/triple-tac-toe/:path*",
        destination: `https://triple-tac-toe.vidhan.io/:path*'`,
      },
      {
        source: "/resume/:path*",
        destination: `https://resume.vidhan.io/:path*'`,
      },
    ];
  },
};
