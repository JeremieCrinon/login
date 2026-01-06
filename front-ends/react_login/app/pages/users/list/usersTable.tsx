import { useTranslation } from "react-i18next"
import type { User } from "~/types/user";

// ShadCn/UI imports
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "~/components/ui/table"

export default function UsersTable({users}: {users: User[]}) {

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

      {users.map((user) => (
        <TableRow key={user.id}>
          <TableCell>{user.id}</TableCell>
          <TableCell>{user.email}</TableCell>
          <TableCell>{user.roles}</TableCell>
        </TableRow>
      ))}

      </TableBody>
    </Table>
  </div>

}
