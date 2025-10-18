"use client";

export function AuthTemplate({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <main className="mx-auto max-w-xl px-4 py-8">
      <h1 className="mb-6 text-2xl font-semibold">{title}</h1>
      {children}
    </main>
  );
}
