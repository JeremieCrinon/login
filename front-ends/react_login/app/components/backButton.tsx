import type React from "react";
import { Button } from "./ui/button";
import { Link } from "react-router";
import { ChevronLeft } from "lucide-react";

export function BackButton({ children, link }: { children: React.ReactNode; link: string}) {
  return (
  <Link to={link}>
    <Button className="absolute mt-5"><ChevronLeft />{children}</Button>
  </Link>
  )
}
