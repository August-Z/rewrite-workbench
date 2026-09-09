import { Button } from "@example/ui";
const Wrapped = memo(Button);
const Assigned = Button;
const { Button: Destructured } = library;
const { Button: Required } = require("@example/ui");
let Later;
Later = Button;
function Local() { return null; }
<><Wrapped /><Assigned /><Destructured /><Required /><Later /><Local /><Button /></>;
