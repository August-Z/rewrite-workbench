import { Button as B } from "@example/ui";
const outer = <B />;
{
  const B = () => null;
  function Nested() { return <B />; }
}
const after = <B />;
