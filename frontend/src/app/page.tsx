import { css } from "styled-system/css";

export default async function Home() {
  return (
    <div>
      <div className={css({ backgroundColor: "red", display: "inline" })}>
        A
      </div>
      <div
        className={css({ backgroundColor: "orange.300", display: "inline" })}
      >
        B
      </div>
    </div>
  );
}
