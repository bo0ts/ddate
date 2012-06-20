cmake_minimum_required(VERSION 2.8)

project(ddate C)

add_executable(ddate ddate.c)

add_custom_command( OUTPUT ddate.1.gz
  COMMAND gzip -c -9 ${CMAKE_SOURCE_DIR}/ddate.1 >> ${CMAKE_CURRENT_BINARY_DIR}/ddate.1.gz
  DEPENDS ${CMAKE_SOURCE_DIR}/ddate.1
  COMMENT "gzipping manpage"
  )

add_custom_target( manpage ALL
  DEPENDS ddate.1.gz
  )

install(FILES ${CMAKE_CURRENT_BINARY_DIR}/ddate.1.gz DESTINATION share/man/man1)
install(TARGETS ddate DESTINATION bin)
