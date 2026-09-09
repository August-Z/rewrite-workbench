import type { Button as T } from "@example/ui";
import { type Button as Inline, Button as Value } from "@example/ui";
import type DefaultType from "@example/ui";
import type * as Types from "@example/ui";
<T />;
<Inline />;
<DefaultType />;
<Types />;
<Types.Button />;
<Value />;
function parameter(T: Component) { return <T />; }
{ const Inline = Local; <Inline />; }
<T />;
