unit ASICPrompter;

{$mode ObjFPC}{$H+}

interface

uses
  Classes, SysUtils, Forms, Controls, Graphics, Dialogs, ExtCtrls, StdCtrls,
  Buttons, ComCtrls, ASICTools;

{$IFDEF UNIX}
  const
    LINEBREAK = #10;
{$ELSE}
  const
    LINEBREAK = #13#10;
{$ENDIF}

type

  { TPrompter }

  TPrompter = class(TForm)
    Option3Image: TImage;
    Option2Image: TImage;
    Option1Image: TImage;
    Logo: TImage;
    Main: TPanel;
    MainPanel: TPanel;
    BottomPanel: TPanel;
    Info: TPanel;
    Option3Panel: TPanel;
    Option2Panel: TPanel;
    Option2Text: TStaticText;
    Option3Text: TStaticText;
    Option1Panel: TPanel;
    LogoHeader: TPanel;
    Selection: TPanel;
    ResizePanel: TPanel;
    Selector: TPanel;
    Header: TStaticText;
    ResizeText: TStaticText;
    Information: TStaticText;
    Option1Text: TStaticText;
    Highlighter: TTimer;
    ToolText3: TStaticText;
    ToolText4: TStaticText;
    ToolText5: TStaticText;
    ToolText2: TStaticText;
    ToolText1: TStaticText;
    TopPanel: TPanel;
    ToolPanel1: TPanel;
    ToolPanel2: TPanel;
    ToolPanel3: TPanel;
    ToolPanel4: TPanel;
    procedure FormActivate(Sender: TObject);
    procedure FormClose(Sender: TObject; var CloseAction: TCloseAction);
    procedure FormCreate(Sender: TObject);
    procedure FormKeyDown(Sender: TObject; var Key: Word; Shift: TShiftState);
    procedure FormMouseDown(Sender: TObject; Button: TMouseButton;
      Shift: TShiftState; X, Y: Integer);
    procedure FormMouseEnter(Sender: TObject);
    procedure FormMouseMove(Sender: TObject; Shift: TShiftState; X, Y: Integer);
    procedure FormMouseUp(Sender: TObject; Button: TMouseButton;
      Shift: TShiftState; X, Y: Integer);
    procedure FormResize(Sender: TObject);
    procedure FormShow(Sender: TObject);
    procedure HighlighterTimer(Sender: TObject);
    procedure Activate(Sender: TObject);
    procedure Option1MouseEnter(Sender: TObject);
    procedure Option1MouseLeave(Sender: TObject);
    procedure Option2MouseEnter(Sender: TObject);
    procedure Option2MouseLeave(Sender: TObject);
    procedure Option3MouseEnter(Sender: TObject);
    procedure Option3MouseLeave(Sender: TObject);
    procedure Option1Click(Sender: TObject);
    procedure Option2Click(Sender: TObject);
    procedure Option3Click(Sender: TObject);
    procedure ResizeMouseDown(Sender: TObject; Button: TMouseButton;
      Shift: TShiftState; X, Y: Integer);
    procedure ResizeMouseMove(Sender: TObject; Shift: TShiftState; X,
      Y: Integer);
    procedure ResizeMouseUp(Sender: TObject; Button: TMouseButton;
      Shift: TShiftState; X, Y: Integer);
    procedure Tool1MouseEnter(Sender: TObject);
    procedure Tool1MouseLeave(Sender: TObject);
    procedure Tool1Click(Sender: TObject);
    procedure Tool2MouseEnter(Sender: TObject);
    procedure Tool2MouseLeave(Sender: TObject);
    procedure Tool2Click(Sender: TObject);
    procedure Tool3MouseEnter(Sender: TObject);
    procedure Tool3MouseLeave(Sender: TObject);
    procedure Tool3Click(Sender: TObject);
    procedure Tool4MouseEnter(Sender: TObject);
    procedure Tool4MouseLeave(Sender: TObject);
    procedure Tool4Click(Sender: TObject);
  private
    FormDragging, FormResizing : Boolean;
    DragX, DragY, ResizeX, ResizeY : Integer;
    Highlighting : Boolean;
  public
    procedure StartHighlight(Sender : TObject);
    procedure StopHighlight(Sender : TObject);
  end;

var
  Prompter: TPrompter;

implementation

{$R *.lfm}

{ TPrompter }

procedure TPrompter.StartHighlight(Sender : TObject);
begin
  Self.Highlighting := True;
  if Self.Color = clCream then
    Self.Color := RGBToColor(34, 34, 34)
  else
    Self.Color := clCream;
end;

procedure TPrompter.StopHighlight(Sender : TObject);
begin
  Self.Highlighting := False;
  Self.Color := RGBToColor(34, 34, 34);
end;

procedure TPrompter.FormClose(Sender: TObject; var CloseAction: TCloseAction);
begin

end;

procedure TPrompter.FormActivate(Sender: TObject);
begin
  if Self.Highlighting then
    Self.StopHighlight(Sender);
end;

procedure TPrompter.FormCreate(Sender: TObject);
begin

end;

procedure TPrompter.FormKeyDown(Sender: TObject; var Key: Word;
  Shift: TShiftState);
begin

end;

procedure TPrompter.FormMouseDown(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  if Button = mbLeft then
  begin
    FormDragging := True;
    DragX := X;
    DragY := Y;
  end;
end;

procedure TPrompter.FormMouseEnter(Sender: TObject);
begin
  Self.FormActivate(Sender);
end;

procedure TPrompter.FormMouseMove(Sender: TObject; Shift: TShiftState; X,
  Y: Integer);
begin
  if FormDragging then
  begin
    Self.SetTempCursor(crSizeAll);
    Self.Left := Self.Left + X - DragX;
    Self.Top := Self.Top + Y - DragY;
  end;
end;

procedure TPrompter.FormMouseUp(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  FormDragging := False;
  DragX := 0;
  DragY := 0;
end;

procedure TPrompter.FormResize(Sender: TObject);
begin
  Self.Option1Panel.Width := (Self.Selection.Width - 192) div 3;
  Self.Option2Panel.Width := (Self.Selection.Width - 192) div 3;
  Self.Option3Panel.Width := (Self.Selection.Width - 192) div 3;
end;

procedure TPrompter.FormShow(Sender: TObject);
begin
  Self.FormResize(Sender);
end;

procedure TPrompter.HighlighterTimer(Sender: TObject);
begin
  if Highlighting then
    Self.StartHighlight(Sender)
  else
    Self.StopHighlight(Sender);
end;

procedure TPrompter.Activate(Sender: TObject);
begin
  Self.StopHighlight(Sender);
end;

procedure TPrompter.Option1MouseEnter(Sender: TObject);
begin
  Self.Information.Caption := '';
end;

procedure TPrompter.Option1MouseLeave(Sender: TObject);
begin
  Self.Information.Caption := LINEBREAK + 'Hover over an option for more information...';
  Self.FormResize(Sender);
end;

procedure TPrompter.Option2MouseEnter(Sender: TObject);
begin
  Self.Information.Caption := '';
end;

procedure TPrompter.Option2MouseLeave(Sender: TObject);
begin
  Self.Information.Caption := LINEBREAK + 'Hover over an option for more information...';
  Self.FormResize(Sender);
end;

procedure TPrompter.Option3MouseEnter(Sender: TObject);
begin
  Self.Information.Caption := '';
end;

procedure TPrompter.Option3MouseLeave(Sender: TObject);
begin
  Self.Information.Caption := LINEBREAK + 'Hover over an option for more information...';
  Self.FormResize(Sender);
end;

procedure TPrompter.Option1Click(Sender: TObject);
begin

end;

procedure TPrompter.Option2Click(Sender: TObject);
begin
  Self.Hide();
  if Tools.ShowModal() = mrYes then
  begin

  end;
  Self.Show();
end;

procedure TPrompter.Option3Click(Sender: TObject);
begin

end;

procedure TPrompter.ResizeMouseDown(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  if Button = mbLeft then
  begin
    FormResizing := True;
    ResizeX := X;
    ResizeY := Y;
  end;
end;

procedure TPrompter.ResizeMouseMove(Sender: TObject; Shift: TShiftState; X,
  Y: Integer);
begin
  if FormResizing then
  begin
    Self.SetTempCursor(crSizeAll);
    if Self.Width + X - ResizeX >= 144 then
       Self.Width := Self.Width + X - ResizeX;
    if Self.Height + Y - ResizeY >= 144 then
       Self.Height := Self.Height + Y - ResizeY;
  end;
end;

procedure TPrompter.ResizeMouseUp(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  FormResizing := False;
  ResizeX := 0;
  ResizeY := 0;
end;

procedure TPrompter.Tool1MouseEnter(Sender: TObject);
begin
  Self.ToolPanel1.Color := clSkyBlue;
end;

procedure TPrompter.Tool1MouseLeave(Sender: TObject);
begin
  Self.ToolPanel1.Color := clCream;
end;

procedure TPrompter.Tool1Click(Sender: TObject);
begin

end;

procedure TPrompter.Tool2MouseEnter(Sender: TObject);
begin
  Self.ToolPanel2.Color := clSkyBlue;
end;

procedure TPrompter.Tool2MouseLeave(Sender: TObject);
begin
  Self.ToolPanel2.Color := clCream;
end;

procedure TPrompter.Tool2Click(Sender: TObject);
begin
  Self.WindowState := wsMinimized;
  Self.UpdateWindowState();
end;

procedure TPrompter.Tool3MouseEnter(Sender: TObject);
begin
  Self.ToolPanel3.Color := clSkyBlue;
end;

procedure TPrompter.Tool3MouseLeave(Sender: TObject);
begin
  Self.ToolPanel3.Color := clCream;
end;

procedure TPrompter.Tool3Click(Sender: TObject);
begin
  if Self.WindowState = wsMaximized then
    Self.WindowState := wsNormal
  else if Self.WindowState = wsNormal then
    Self.WindowState := wsMaximized;
  Self.UpdateWindowState();
end;

procedure TPrompter.Tool4MouseEnter(Sender: TObject);
begin
  Self.ToolPanel4.Color := clSkyBlue;
end;

procedure TPrompter.Tool4MouseLeave(Sender: TObject);
begin
  Self.ToolPanel4.Color := clCream;
end;

procedure TPrompter.Tool4Click(Sender: TObject);
begin
  Self.Close();
end;

end.

