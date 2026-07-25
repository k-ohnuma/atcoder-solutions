import "server-only";
import z from "zod";

type AppConfig = {
  apiBaseEndpoint: string;
};

type ServerConfig = {
  appConfig: AppConfig;
};

const envSchema = z.object({
  API_BASE_ENDPOINT: z.string(),
});
const env = envSchema.parse({
  API_BASE_ENDPOINT: process.env.API_BASE_ENDPOINT,
});

export const serverConfig: ServerConfig = {
  appConfig: {
    apiBaseEndpoint: env.API_BASE_ENDPOINT,
  },
};
