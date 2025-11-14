import { getApps, initializeApp } from "firebase/app";
import { Auth, getAuth } from "firebase/auth";
import { clientConfig } from "../config/client";

const getFirebaseApp = () => {
  if (getApps().length >= 1) {
    return getApps()[0];
  }
  const config = clientConfig.firebaseConfig;
  return initializeApp({
    apiKey: config.apiKey,
    authDomain: config.authDomain,
    projectId: config.projectId,
    appId: config.appId,
    messagingSenderId: config.messagingSenderId,
    storageBucket: config.storageBucket,
  });
};

let auth: Auth;
export const getFirebaseAuth = () => {
  if (!auth) {
    auth = getAuth(getFirebaseApp());
  }
  return auth;
};
