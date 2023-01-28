module default {
 type User {
    required property firstName -> str;
    required property lastName -> str;
    property age -> int32;
  }

   type Favorite {
    property food -> str;
    multi link favorites -> User;
  }
}
