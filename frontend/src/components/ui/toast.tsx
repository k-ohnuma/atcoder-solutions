"use client";

import { CheckCircle2, XCircle } from "lucide-react";
import { createContext, ReactNode, useCallback, useContext, useMemo, useState } from "react";
import { cn } from "@/lib/utils";

type ToastVariant = "success" | "error";

type ToastPayload = {
  title: string;
  description?: string;
  variant?: ToastVariant;
  durationMs?: number;
};

type ToastItem = ToastPayload & {
  id: number;
};

type ToastContextValue = {
  toast: (payload: ToastPayload) => void;
};

const ToastContext = createContext<ToastContextValue | null>(null);

let toastId = 0;

export function ToastProvider({ children }: { children: ReactNode }) {
  const [items, setItems] = useState<ToastItem[]>([]);

  const remove = useCallback((id: number) => {
    setItems((prev) => prev.filter((item) => item.id !== id));
  }, []);

  const toast = useCallback(
    ({ title, description, variant = "success", durationMs = 3000 }: ToastPayload) => {
      const id = ++toastId;
      setItems((prev) => [...prev, { id, title, description, variant, durationMs }]);
      setTimeout(() => remove(id), durationMs);
    },
    [remove],
  );

  const value = useMemo(() => ({ toast }), [toast]);

  return (
    <ToastContext.Provider value={value}>
      {children}
      <div className="pointer-events-none fixed right-4 bottom-4 z-[100] flex w-[min(92vw,360px)] flex-col gap-2">
        {items.map((item) => (
          <div
            key={item.id}
            className={cn(
              "pointer-events-auto rounded-lg border bg-background p-3 shadow-lg",
              item.variant === "error" && "border-destructive/50",
            )}
            role="status"
            aria-live="polite"
          >
            <div className="flex items-start gap-2">
              {item.variant === "error" ? (
                <XCircle className="mt-0.5 size-4 text-destructive" />
              ) : (
                <CheckCircle2 className="mt-0.5 size-4 text-emerald-600" />
              )}
              <div className="min-w-0">
                <p className="text-sm font-medium">{item.title}</p>
                {item.description ? <p className="mt-1 text-xs text-muted-foreground">{item.description}</p> : null}
              </div>
            </div>
          </div>
        ))}
      </div>
    </ToastContext.Provider>
  );
}

export function useToast() {
  const context = useContext(ToastContext);
  if (!context) {
    throw new Error("useToast must be used within ToastProvider");
  }
  return context;
}
