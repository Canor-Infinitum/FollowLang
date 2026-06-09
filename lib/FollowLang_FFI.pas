unit FollowLang_FFI;

{$mode objfpc}{$H+}

interface

uses
  Classes, SysUtils;

function canoros_spawn_module(module_type: DWord): QWord; cdecl; external 'followlang.dll';
function canoros_get_module_info(
  id: QWord;
  out_type: PDWord;
  out_pre, out_peri, out_post: PInteger;
  name_buf: PChar; name_max: DWord;
  status_buf: PChar; status_max: DWord
): DWord; cdecl; external 'followlang.dll';
function canoros_parse_and_eval(
  id: QWord;
  code_ptr: PChar; code_len: DWord;
  out_buf: PChar; out_max: DWord
): DWord; cdecl; external 'followlang.dll';
function canoros_eval_project(
  code_ptr: PChar; code_len: DWord;
  out_buf: PChar; out_max: DWord
): DWord; cdecl; external 'followlang.dll';

function instinct_correlate_oeis(
  project_info_ptr: PChar; project_info_len: DWord;
  db_info_ptr: PChar; db_info_len: DWord;
  out_buf: PChar; out_max: DWord
): DWord; cdecl; external 'followerinstinct.dll';

function fdb_simulate_query(
  query_ptr: PChar; query_len: DWord;
  out_buf: PChar; out_max: DWord
): DWord; cdecl; external 'followdb.dll';

implementation

end.
