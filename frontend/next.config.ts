import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  webpack: (config) => {
    config.output.webassemblyModuleFilename = "static/wasm/[modulehash].wasm";
    config.experiments = {
      asyncWebAssembly: true,
      layers: true,

      ...config.experiments
    };

    return config;
  },
};

export default nextConfig;
