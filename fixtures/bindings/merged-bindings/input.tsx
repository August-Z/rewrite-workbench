const Before = Local;
import { Button, Button as Before, Button as After, Button as Augmented, Button as Overload, Button as Shape, Button as EnumLike, Button as ClassLike } from "@example/ui";
const After = Local;
namespace Augmented { export const part = 1; }
function Overload(): unknown;
interface Shape {}
enum EnumLike { A }
class ClassLike {}
import type { Button as Typed } from "@example/ui";
const Typed = Local;
<><Before /><After /><Augmented /><Overload /><Shape /><EnumLike /><ClassLike /><Typed /><Button /></>;
