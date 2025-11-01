import "server-only";
import { cert, getApps, initializeApp } from "firebase-admin/app";
import { serverConfig } from "../config/backend";
import { getAuth } from "firebase-admin/auth";

export const firebaseAdmin =
  getApps()[0] ??
  initializeApp({
    credential: cert({
      projectId: serverConfig.firebaseConfig.projectId,
      clientEmail: serverConfig.firebaseConfig.clientEmail,
      privateKey: serverConfig.firebaseConfig.privateKey,
    }),
  });

export const auth = getAuth();
