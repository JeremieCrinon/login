import { Controller, Get, Post, Body, Put, Param, Delete, UseGuards } from '@nestjs/common';
import { UserService } from './user.service';
import { CreateUserDto } from './dto/create-user.dto';
import { EditUserRoleDto } from './dto/edit-user-role.dto';
import { AuthGuard } from 'src/auth/guards/auth.guard';
import { RequiredRole } from 'src/auth/decorators/roles.decorator';
import { Role } from './entities/user.entity';

@Controller('users')
@UseGuards(AuthGuard)
@RequiredRole(Role.EDIT_USERS)
export class UserController {
  constructor(private readonly userService: UserService) { }

  @Post('new/:lang')
  create(@Body() createUserDto: CreateUserDto) {
    return this.userService.create(createUserDto);
  }

  @Get('list-roles')
  listRoles() {
    return this.userService.listRoles();
  }

  @Get()
  findAll() {
    return this.userService.findAll();
  }

  @Get(':id')
  findOne(@Param('id') id: string) {
    return this.userService.findOne(+id);
  }

  @Put(':id/roles')
  update(@Param('id') id: string, @Body() editUserRoleDto: EditUserRoleDto) {
    return this.userService.updateRole(+id, editUserRoleDto);
  }

  @Delete(':id')
  remove(@Param('id') id: string) {
    return this.userService.remove(+id);
  }
}
