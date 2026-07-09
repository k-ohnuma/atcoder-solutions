import Link from "next/link";
import { ComponentProps } from "react";
import { Button } from "@/components/ui/button";

type ButtonLinkProps = ComponentProps<typeof Link> & Pick<ComponentProps<typeof Button>, "variant" | "size" | "className">;

export function ButtonLink({ variant, size, className, children, ...linkProps }: ButtonLinkProps) {
  return (
    <Button asChild variant={variant} size={size} className={className}>
      <Link {...linkProps}>{children}</Link>
    </Button>
  );
}
