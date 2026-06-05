program followlang;

{$mode objfpc}{$H+}

uses
  {$IFDEF UNIX}
  cthreads,
  {$ENDIF}
  {$IFDEF HASAMIGA}
  athreads,
  {$ENDIF}
  Interfaces,
  Forms, ASICLogin, ASICPrompter, ASICWorkshop, ASICTools, ASICHighlightReel;

{$R *.res}

begin
  RequireDerivedFormResource:=True;
  Application.Scaled:=True;
  {$PUSH}{$WARN 5044 OFF}
  Application.MainFormOnTaskbar:=True;
  {$POP}
  Application.Initialize;
  Application.CreateForm(TLogin, Login);
  Application.CreateForm(TPrompter, Prompter);
  Application.CreateForm(TTools, Tools);
  Application.CreateForm(THighlightReel, HighlightReel);
  Application.CreateForm(TWorkshop, Workshop);
  Application.Run;
end.

