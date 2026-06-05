unit ASICWorkshop;

{$mode ObjFPC}{$H+}

interface

uses
  Classes, SysUtils, Forms, Controls, Graphics, Dialogs, ExtCtrls, StdCtrls,
  Buttons, ComCtrls, Menus, ASICHighlightReel;

type

  { TWorkshop }

  TWorkshop = class(TForm)
    Logo: TImage;
    Main: TPanel;
    MainPanel: TPanel;
    LeftPanel: TPanel;
    BottomPanel: TPanel;
    LogoHeader: TPanel;
    MainPopupMenu: TPopupMenu;
    ActivateItem: TMenuItem;
    NewServiceItem: TMenuItem;
    NewWorkerItem: TMenuItem;
    Separator1: TMenuItem;
    StatusText: TStaticText;
    Status: TPanel;
    ResizePanel: TPanel;
    Selector: TPanel;
    LeftSplitter: TSplitter;
    Header: TStaticText;
    ResizeText: TStaticText;
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
    procedure ActivateItemClick(Sender: TObject);
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
  Workshop: TWorkshop;

implementation

{$R *.lfm}

{ TWorkshop }

procedure TWorkshop.StartHighlight(Sender : TObject);
begin
  Self.Highlighting := True;
  if not (Self.Main.BevelColor = RGBToColor(34, 34, 34)) then
    Self.Main.BevelColor := RGBToColor(34, 34, 34)
  else
    Self.Main.BevelColor := RGBToColor(255, 136, 68);

  if not (Self.MainPanel.BevelColor = RGBToColor(34, 34, 34)) then
    Self.MainPanel.BevelColor := RGBToColor(34, 34, 34)
  else
    Self.MainPanel.BevelColor := RGBToColor(255, 136, 68);

  if not (Self.LogoHeader.BevelColor = RGBToColor(34, 34, 34)) then
    Self.LogoHeader.BevelColor := RGBToColor(34, 34, 34)
  else
    Self.LogoHeader.BevelColor := RGBToColor(255, 136, 68);

  if not (Self.ToolPanel1.BevelColor = RGBToColor(34, 34, 34)) then
    Self.ToolPanel1.BevelColor := RGBToColor(34, 34, 34)
  else
    Self.ToolPanel1.BevelColor := RGBToColor(255, 136, 68);

  if not (Self.ToolPanel2.BevelColor = RGBToColor(34, 34, 34)) then
    Self.ToolPanel2.BevelColor := RGBToColor(34, 34, 34)
  else
    Self.ToolPanel2.BevelColor := RGBToColor(255, 136, 68);

  if not (Self.ToolPanel3.BevelColor = RGBToColor(34, 34, 34)) then
    Self.ToolPanel3.BevelColor := RGBToColor(34, 34, 34)
  else
    Self.ToolPanel3.BevelColor := RGBToColor(255, 136, 68);

  if not (Self.ToolPanel4.BevelColor = RGBToColor(34, 34, 34)) then
    Self.ToolPanel4.BevelColor := RGBToColor(34, 34, 34)
  else
    Self.ToolPanel4.BevelColor := RGBToColor(255, 136, 68);

  if not (Self.LeftSplitter.Color = clCream) then
    Self.LeftSplitter.Color := clCream
  else
    Self.LeftSplitter.Color := RGBToColor(34, 34, 34);
end;

procedure TWorkshop.StopHighlight(Sender : TObject);
begin
  Self.Highlighting := False;
  Self.Main.BevelColor := RGBToColor(255, 136, 68);
  Self.MainPanel.BevelColor := RGBToColor(255, 136, 68);
  Self.LogoHeader.BevelColor := RGBToColor(255, 136, 68);
  Self.ToolPanel1.BevelColor := RGBToColor(255, 136, 68);
  Self.ToolPanel2.BevelColor := RGBToColor(255, 136, 68);
  Self.ToolPanel3.BevelColor := RGBToColor(255, 136, 68);
  Self.ToolPanel4.BevelColor := RGBToColor(255, 136, 68);
  Self.LeftSplitter.Color := RGBToColor(34, 34, 34);
end;

procedure TWorkshop.FormClose(Sender: TObject; var CloseAction: TCloseAction);
begin

end;

procedure TWorkshop.FormActivate(Sender: TObject);
begin
  if Self.Highlighting then
    Self.StopHighlight(Sender);
end;

procedure TWorkshop.ActivateItemClick(Sender: TObject);
begin
  HighlightReel.Show();
end;

procedure TWorkshop.FormCreate(Sender: TObject);
begin
  Self.LeftPanel.Width := 0;
end;

procedure TWorkshop.FormKeyDown(Sender: TObject; var Key: Word;
  Shift: TShiftState);
begin

end;

procedure TWorkshop.FormMouseDown(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  if Button = mbLeft then
  begin
    FormDragging := True;
    DragX := X;
    DragY := Y;
  end;
end;

procedure TWorkshop.FormMouseEnter(Sender: TObject);
begin
  Self.FormActivate(Sender);
end;

procedure TWorkshop.FormMouseMove(Sender: TObject; Shift: TShiftState; X,
  Y: Integer);
begin
  if FormDragging then
  begin
    Self.SetTempCursor(crSizeAll);
    Self.Left := Self.Left + X - DragX;
    Self.Top := Self.Top + Y - DragY;
  end;
end;

procedure TWorkshop.FormMouseUp(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  FormDragging := False;
  DragX := 0;
  DragY := 0;
end;

procedure TWorkshop.FormShow(Sender: TObject);
begin

end;

procedure TWorkshop.HighlighterTimer(Sender: TObject);
begin
  if Highlighting then
    Self.StartHighlight(Sender)
  else
    Self.StopHighlight(Sender);
end;

procedure TWorkshop.ResizeMouseDown(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  if Button = mbLeft then
  begin
    FormResize := True;
    ResizeX := X;
    ResizeY := Y;
  end;
end;

procedure TWorkshop.ResizeMouseMove(Sender: TObject; Shift: TShiftState; X,
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

procedure TWorkshop.ResizeMouseUp(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  FormResize := False;
  ResizeX := 0;
  ResizeY := 0;
end;

procedure TWorkshop.Tool1MouseEnter(Sender: TObject);
begin
  Self.ToolPanel1.Color := clSkyBlue;
end;

procedure TWorkshop.Tool1MouseLeave(Sender: TObject);
begin
  Self.ToolPanel1.Color := clCream;
end;

procedure TWorkshop.Tool1Click(Sender: TObject);
begin
  Self.MainPopupMenu.Popup();
end;

procedure TWorkshop.Tool2MouseEnter(Sender: TObject);
begin
  Self.ToolPanel2.Color := clSkyBlue;
end;

procedure TWorkshop.Tool2MouseLeave(Sender: TObject);
begin
  Self.ToolPanel2.Color := clCream;
end;

procedure TWorkshop.Tool2Click(Sender: TObject);
begin
  Self.WindowState := wsMinimized;
  Self.UpdateWindowState();
end;

procedure TWorkshop.Tool3MouseEnter(Sender: TObject);
begin
  Self.ToolPanel3.Color := clSkyBlue;
end;

procedure TWorkshop.Tool3MouseLeave(Sender: TObject);
begin
  Self.ToolPanel3.Color := clCream;
end;

procedure TWorkshop.Tool3Click(Sender: TObject);
begin
  if Self.WindowState = wsMaximized then
    Self.WindowState := wsNormal
  else if Self.WindowState = wsNormal then
    Self.WindowState := wsMaximized;
  Self.UpdateWindowState();
end;

procedure TWorkshop.Tool4MouseEnter(Sender: TObject);
begin
  Self.ToolPanel4.Color := clSkyBlue;
end;

procedure TWorkshop.Tool4MouseLeave(Sender: TObject);
begin
  Self.ToolPanel4.Color := clCream;
end;

procedure TWorkshop.Tool4Click(Sender: TObject);
begin
  Self.Close();
end;

end.

