import { Button, Button as B, Link as Other } from "@example/ui";
import { Button as Foreign } from "@example/other";
import { Button as Barrel } from "./barrel";
import { Button as Escaped } from "@example/\u0075i";
import { "Button" as Quoted } from "@example/ui";
import { Button as Prefix } from "@example/ui/subpath";
export { Button as PublicButton };
<><Button size="small"></Button><B /><Foreign /><Other /><Barrel /><Escaped /><Quoted /><Prefix /></>;
