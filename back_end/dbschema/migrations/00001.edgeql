CREATE MIGRATION m1wjcrfwpf3gyopcegjhxtughawexysv34fu53mpyhjeqcrf7yl66a
    ONTO initial
{
  CREATE FUTURE nonrecursive_access_policies;
  CREATE TYPE default::User {
      CREATE PROPERTY age -> std::int32;
      CREATE REQUIRED PROPERTY firstName -> std::str;
      CREATE REQUIRED PROPERTY lastName -> std::str;
  };
  CREATE TYPE default::Favorite {
      CREATE MULTI LINK actors -> default::User;
      CREATE PROPERTY food -> std::str;
  };
};
