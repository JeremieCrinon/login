import { useTranslation } from "react-i18next"

// ShadCn/UI imports
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "~/components/ui/table";
import { Skeleton } from "~/components/ui/skeleton";

export default function UsersTableSkeleton() {

  const { t } = useTranslation();

  return <div className="ml-5">
    <Table>

      <TableHeader>
        <TableRow>
          <TableHead>{t("id")}</TableHead>
          <TableHead>{t("email")}</TableHead>
          <TableHead>{t("roles")}</TableHead>
          <TableHead>{t("edit")}</TableHead>
          <TableHead>{t("delete")}</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>

      {Array.from({ length: 10 }).map((_, index) => (
        <TableRow key={index}>
          <TableCell><Skeleton className="w-5 h-5" /></TableCell>
          <TableCell><Skeleton className="w-30 h-5" /></TableCell>
          <TableCell><Skeleton className="w-20 h-5" /></TableCell>
          <TableCell><Skeleton className="w-10 h-10" /></TableCell>
          <TableCell><Skeleton className="w-10 h-10" /></TableCell>
        </TableRow>
      ))}

      </TableBody>
    </Table>
  </div>

}
