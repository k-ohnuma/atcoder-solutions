"use client";

import { useEffect, useRef } from "react";
import { clientConfig } from "@/shared/config/client";

type TurnstileFieldProps = {
  onTokenChangeAction: (token: string | null) => void;
};

type Turnstile = {
  render: (
    container: HTMLElement,
    options: {
      sitekey: string;
      theme: "light" | "dark" | "auto";
      callback: (token: string) => void;
      "expired-callback": () => void;
      "error-callback": () => void;
    },
  ) => string;
  remove: (widgetId: string) => void;
};

declare global {
  interface Window {
    turnstile?: Turnstile;
  }
}

const SCRIPT_ID = "cloudflare-turnstile-script";
const SCRIPT_SRC = "https://challenges.cloudflare.com/turnstile/v0/api.js?render=explicit";
let turnstileScriptPromise: Promise<void> | null = null;

function resetTurnstileScriptLoad() {
  turnstileScriptPromise = null;
  document.getElementById(SCRIPT_ID)?.remove();
}

function loadTurnstileScript() {
  if (window.turnstile) {
    return Promise.resolve();
  }
  if (turnstileScriptPromise) {
    return turnstileScriptPromise;
  }

  const existingScript = document.getElementById(SCRIPT_ID) as HTMLScriptElement | null;
  if (existingScript) {
    turnstileScriptPromise = new Promise<void>((resolve, reject) => {
      existingScript.addEventListener("load", () => resolve(), { once: true });
      existingScript.addEventListener(
        "error",
        () => {
          resetTurnstileScriptLoad();
          reject(new Error("failed to load Turnstile script"));
        },
        { once: true },
      );
    });
    return turnstileScriptPromise;
  }

  turnstileScriptPromise = new Promise<void>((resolve, reject) => {
    const script = document.createElement("script");
    script.id = SCRIPT_ID;
    script.src = SCRIPT_SRC;
    script.async = true;
    script.defer = true;
    script.onload = () => resolve();
    script.onerror = () => {
      resetTurnstileScriptLoad();
      reject(new Error("failed to load Turnstile script"));
    };
    document.body.appendChild(script);
  });
  return turnstileScriptPromise;
}

export function TurnstileField({ onTokenChangeAction }: TurnstileFieldProps) {
  const containerRef = useRef<HTMLDivElement | null>(null);
  const widgetIdRef = useRef<string | null>(null);
  const { enabled, siteKey } = clientConfig.turnstileConfig;

  useEffect(() => {
    if (!enabled || !siteKey || !containerRef.current) {
      return;
    }

    let disposed = false;

    loadTurnstileScript()
      .then(() => {
        if (disposed || !window.turnstile || !containerRef.current) {
          return;
        }
        widgetIdRef.current = window.turnstile.render(containerRef.current, {
          sitekey: siteKey,
          theme: "light",
          callback: (token) => onTokenChangeAction(token),
          "expired-callback": () => onTokenChangeAction(null),
          "error-callback": () => onTokenChangeAction(null),
        });
      })
      .catch(() => onTokenChangeAction(null));

    return () => {
      disposed = true;
      if (widgetIdRef.current && window.turnstile) {
        window.turnstile.remove(widgetIdRef.current);
      }
      widgetIdRef.current = null;
      onTokenChangeAction(null);
    };
  }, [enabled, siteKey, onTokenChangeAction]);

  if (!enabled) {
    return null;
  }

  return <div ref={containerRef} />;
}
