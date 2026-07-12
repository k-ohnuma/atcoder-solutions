type AuthFormErrorProps = {
  message?: string;
};

export function AuthFormError({ message }: AuthFormErrorProps) {
  if (!message) {
    return null;
  }

  return <p className="rounded-md border border-destructive/50 bg-destructive/5 p-3 text-sm text-destructive">{message}</p>;
}
