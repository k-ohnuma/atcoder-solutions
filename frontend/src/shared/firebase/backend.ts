import "server-only";
import { cert, getApps, initializeApp } from "firebase-admin/app";
import { backendConfig } from "../config/backend";
import { getAuth } from "firebase-admin/auth";

export const firebaseAdmin = 
  getApps()[0] ?? 
  initializeApp({
    credential: cert({
      projectId: backendConfig.firebaseConfig.projectId,
      clientEmail: backendConfig.firebaseConfig.clientEmail,
      privateKey: backendConfig.firebaseConfig.privateKey
    }),
  });

export const auth = getAuth();
