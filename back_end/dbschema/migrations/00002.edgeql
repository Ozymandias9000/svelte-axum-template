CREATE MIGRATION m1ptlnpoecql6hmwbxriifpd5b5jot2wsgmorvm4fhyip45rpfvgga
    ONTO m1wjcrfwpf3gyopcegjhxtughawexysv34fu53mpyhjeqcrf7yl66a
{
  ALTER TYPE default::Favorite {
      ALTER LINK actors {
          RENAME TO favorites;
      };
  };
};
