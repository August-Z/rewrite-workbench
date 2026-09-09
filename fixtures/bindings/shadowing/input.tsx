import { Button as B } from "@example/ui";
<B />;
function parameter({ B }: Props) { return <B />; }
{
  <B />;
  const B = Local;
  const nested = () => <B />;
}
function hoisted() { <B />; var B = Local; }
try {} catch (B) { <B />; }
const named = function B() { return <B />; };
for (const B of components) { <B />; }
{ class B {} <B />; }
<B />;
