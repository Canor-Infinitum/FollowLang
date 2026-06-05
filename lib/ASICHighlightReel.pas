unit ASICHighlightReel;

{$mode ObjFPC}{$H+}

interface

uses
  Classes, SysUtils, Forms, Controls, Graphics, Dialogs, ExtCtrls, StdCtrls,
  Buttons, ComCtrls;

type

  { THighlightReel }

  THighlightReel = class(TForm)
    Logo: TImage;
    Main: TPanel;
    MainPanel: TPanel;
    BottomPanel: TPanel;
    LogoHeader: TPanel;
    MainReel: TPanel;
    StatusText: TStaticText;
    Status: TPanel;
    SubReel: TPanel;
    ResizePanel: TPanel;
    Selector: TPanel;
    Header: TStaticText;
    ResizeText: TStaticText;
    Highlighter: TTimer;
    ReelSplitter: TSplitter;
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
    procedure FormShow(Sender: TObject);
    procedure HighlighterTimer(Sender: TObject);
    procedure Activate(Sender: TObject);
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
    FormDragging, FormResize : Boolean;
    DragX, DragY, ResizeX, ResizeY : Integer;
    Highlighting : Boolean;
  public
    procedure StartHighlight(Sender : TObject);
    procedure StopHighlight(Sender : TObject);
  end;

var
  HighlightReel: THighlightReel;

implementation

{$R *.lfm}

{ THighlightReel }

procedure THighlightReel.StartHighlight(Sender : TObject);
begin
  Self.Highlighting := True;
  if Self.Color = clCream then
    Self.Color := RGBToColor(34, 34, 34)
  else
    Self.Color := clCream;
end;

procedure THighlightReel.StopHighlight(Sender : TObject);
begin
  Self.Highlighting := False;
  Self.Color := RGBToColor(34, 34, 34);
end;

procedure THighlightReel.FormClose(Sender: TObject; var CloseAction: TCloseAction);
begin

end;

procedure THighlightReel.FormActivate(Sender: TObject);
begin
  if Self.Highlighting then
    Self.StopHighlight(Sender);
end;

procedure THighlightReel.FormCreate(Sender: TObject);
begin

end;

procedure THighlightReel.FormKeyDown(Sender: TObject; var Key: Word;
  Shift: TShiftState);
begin

end;

procedure THighlightReel.FormMouseDown(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  if Button = mbLeft then
  begin
    FormDragging := True;
    DragX := X;
    DragY := Y;
  end;
end;

procedure THighlightReel.FormMouseEnter(Sender: TObject);
begin
  Self.FormActivate(Sender);
end;

procedure THighlightReel.FormMouseMove(Sender: TObject; Shift: TShiftState; X,
  Y: Integer);
begin
  if FormDragging then
  begin
    Self.SetTempCursor(crSizeAll);
    Self.Left := Self.Left + X - DragX;
    Self.Top := Self.Top + Y - DragY;
  end;
end;

procedure THighlightReel.FormMouseUp(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  FormDragging := False;
  DragX := 0;
  DragY := 0;
end;

procedure THighlightReel.FormShow(Sender: TObject);
begin

end;

procedure THighlightReel.HighlighterTimer(Sender: TObject);
begin
  if Highlighting then
    Self.StartHighlight(Sender)
  else
    Self.StopHighlight(Sender);
end;

procedure THighlightReel.Activate(Sender: TObject);
begin
  Self.StopHighlight(Sender);
end;

procedure THighlightReel.ResizeMouseDown(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  if Button = mbLeft then
  begin
    FormResize := True;
    ResizeX := X;
    ResizeY := Y;
  end;
end;

procedure THighlightReel.ResizeMouseMove(Sender: TObject; Shift: TShiftState; X,
  Y: Integer);
begin
  if FormResize then
  begin
    Self.SetTempCursor(crSizeAll);
    if Self.Width + X - ResizeX >= 144 then
       Self.Width := Self.Width + X - ResizeX;
    if Self.Height + Y - ResizeY >= 144 then
       Self.Height := Self.Height + Y - ResizeY;
  end;
end;

procedure THighlightReel.ResizeMouseUp(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  FormResize := False;
  ResizeX := 0;
  ResizeY := 0;
end;

procedure THighlightReel.Tool1MouseEnter(Sender: TObject);
begin
  Self.ToolPanel1.Color := clSkyBlue;
end;

procedure THighlightReel.Tool1MouseLeave(Sender: TObject);
begin
  Self.ToolPanel1.Color := clCream;
end;

procedure THighlightReel.Tool1Click(Sender: TObject);
begin

end;

procedure THighlightReel.Tool2MouseEnter(Sender: TObject);
begin
  Self.ToolPanel2.Color := clSkyBlue;
end;

procedure THighlightReel.Tool2MouseLeave(Sender: TObject);
begin
  Self.ToolPanel2.Color := clCream;
end;

procedure THighlightReel.Tool2Click(Sender: TObject);
begin
  Self.WindowState := wsMinimized;
  Self.UpdateWindowState();
end;

procedure THighlightReel.Tool3MouseEnter(Sender: TObject);
begin
  Self.ToolPanel3.Color := clSkyBlue;
end;

procedure THighlightReel.Tool3MouseLeave(Sender: TObject);
begin
  Self.ToolPanel3.Color := clCream;
end;

procedure THighlightReel.Tool3Click(Sender: TObject);
begin
  if Self.WindowState = wsMaximized then
    Self.WindowState := wsNormal
  else if Self.WindowState = wsNormal then
    Self.WindowState := wsMaximized;
  Self.UpdateWindowState();
end;

procedure THighlightReel.Tool4MouseEnter(Sender: TObject);
begin
  Self.ToolPanel4.Color := clSkyBlue;
end;

procedure THighlightReel.Tool4MouseLeave(Sender: TObject);
begin
  Self.ToolPanel4.Color := clCream;
end;

procedure THighlightReel.Tool4Click(Sender: TObject);
begin
  Self.Close();
end;

end.

