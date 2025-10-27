import "server-only";
import z from "zod";

type FirebaseConfig = {
  projectId: string;
  clientEmail: string;
  privateKey: string;
};

type AppConfig = {
  appName: string;
  apiBaseEndpoint: string;
  appOrigin: string
};

type ServerConfig = {
  firebaseConfig: FirebaseConfig;
  appConfig: AppConfig;
};

const envSchema = z.object({
  FIREBASE_PROJECT_ID: z.string(),
  FIREBASE_CLIENT_EMAIL: z.string(),
  FIREBASE_PRIVATE_KEY: z.string(),
  APP_NAME: z.string(),
  API_BASE_ENDPOINT: z.string(),
  APP_ORIGIN: z.string()
});
const env = envSchema.parse({
  FIREBASE_PROJECT_ID: process.env.FIREBASE_PROJECT_ID,
  FIREBASE_CLIENT_EMAIL: process.env.FIREBASE_CLIENT_EMAIL,
  FIREBASE_PRIVATE_KEY: process.env.FIREBASE_PRIVATE_KEY,
  APP_NAME: process.env.APP_NAME,
  API_BASE_ENDPOINT: process.env.API_BASE_ENDPOINT,
  APP_ORIGIN: process.env.APP_ORIGIN
});

export const serverConfig: ServerConfig = {
  firebaseConfig: {
    projectId: env.FIREBASE_PROJECT_ID,
    clientEmail: env.FIREBASE_CLIENT_EMAIL,
    privateKey: env.FIREBASE_PRIVATE_KEY.replace(/\\n/g, "\n"),
  },
  appConfig: {
    appName: env.APP_NAME,
    apiBaseEndpoint: env.API_BASE_ENDPOINT,
    appOrigin: env.APP_ORIGIN
  },
};
