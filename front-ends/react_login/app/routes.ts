import {
  type RouteConfig,
  route
} from "@react-router/dev/routes";
import { routes } from "./lib/routes";

// WARNING: Do not define routes here, they should be defined in lib/routes.ts

export default Object.values(routes).map((r) => 
  route(r.path, r.component)                                        
) satisfies RouteConfig;
