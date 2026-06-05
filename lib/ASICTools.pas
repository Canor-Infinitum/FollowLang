unit ASICTools;

{$mode ObjFPC}{$H+}

interface

uses
  Classes, SysUtils, Forms, Controls, Graphics, Dialogs, ExtCtrls, StdCtrls,
  Buttons, ComCtrls, Menus, ASICWorkshop, ASICHighlightReel;

type

  { TTools }

  TTools = class(TForm)
    Logo: TImage;
    Main: TPanel;
    MainPanel: TPanel;
    BottomPanel: TPanel;
    LogoHeader: TPanel;
    MainPopupMenu: TPopupMenu;
    NewServiceItem: TMenuItem;
    NewWorkerItem: TMenuItem;
    NewModalItem: TMenuItem;
    StatusText: TStaticText;
    Status: TPanel;
    ResizePanel: TPanel;
    Selector: TPanel;
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
    procedure NewModalItemClick(Sender: TObject);
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
    procedure NewModal(Sender : TObject);
  private
    FormDragging, FormResize : Boolean;
    DragX, DragY, ResizeX, ResizeY : Integer;
    Highlighting : Boolean;
    Modals : array of TWorkshop;
  public
    procedure StartHighlight(Sender : TObject);
    procedure StopHighlight(Sender : TObject);
  end;

var
  Tools: TTools;

implementation

{$R *.lfm}

{ TTools }

procedure TTools.NewModal(Sender : TObject);
begin
  SetLength(Self.Modals, Length(Self.Modals) + 1);
  Self.Modals[High(Self.Modals)] := TWorkshop.Create(Self);
  Self.Modals[High(Self.Modals)].Show();           
  Self.Modals[High(Self.Modals)].StartHighlight(Sender);
end;

procedure TTools.StartHighlight(Sender : TObject);
begin
  Self.Highlighting := True;
  if Self.Color = clCream then
    Self.Color := RGBToColor(34, 34, 34)
  else
    Self.Color := clCream;
end;

procedure TTools.StopHighlight(Sender : TObject);
begin
  Self.Highlighting := False;
  Self.Color := RGBToColor(34, 34, 34);
end;

procedure TTools.FormClose(Sender: TObject; var CloseAction: TCloseAction);
begin
  HighlightReel.Close();
end;

procedure TTools.FormActivate(Sender: TObject);
begin
  if Self.Highlighting then
    Self.StopHighlight(Sender);
end;

procedure TTools.FormCreate(Sender: TObject);
begin
  HighlightReel := THighlightReel.Create(Self);
end;

procedure TTools.FormKeyDown(Sender: TObject; var Key: Word;
  Shift: TShiftState);
begin

end;

procedure TTools.FormMouseDown(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  if Button = mbLeft then
  begin
    FormDragging := True;
    DragX := X;
    DragY := Y;
  end;
end;

procedure TTools.FormMouseEnter(Sender: TObject);
begin
  Self.FormActivate(Sender);
end;

procedure TTools.FormMouseMove(Sender: TObject; Shift: TShiftState; X,
  Y: Integer);
begin
  if FormDragging then
  begin
    Self.SetTempCursor(crSizeAll);
    Self.Left := Self.Left + X - DragX;
    Self.Top := Self.Top + Y - DragY;
  end;
end;

procedure TTools.FormMouseUp(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  FormDragging := False;
  DragX := 0;
  DragY := 0;
end;

procedure TTools.FormShow(Sender: TObject);
begin
  HighlightReel.Show();
end;

procedure TTools.HighlighterTimer(Sender: TObject);
begin
  if Highlighting then
    Self.StartHighlight(Sender)
  else
    Self.StopHighlight(Sender);
end;

procedure TTools.Activate(Sender: TObject);
begin
  Self.StopHighlight(Sender);
end;

procedure TTools.NewModalItemClick(Sender: TObject);
begin
  Self.NewModal(Sender);
end;

procedure TTools.ResizeMouseDown(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  if Button = mbLeft then
  begin
    FormResize := True;
    ResizeX := X;
    ResizeY := Y;
  end;
end;

procedure TTools.ResizeMouseMove(Sender: TObject; Shift: TShiftState; X,
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

procedure TTools.ResizeMouseUp(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  FormResize := False;
  ResizeX := 0;
  ResizeY := 0;
end;

procedure TTools.Tool1MouseEnter(Sender: TObject);
begin
  Self.ToolPanel1.Color := clSkyBlue;
end;

procedure TTools.Tool1MouseLeave(Sender: TObject);
begin
  Self.ToolPanel1.Color := clCream;
end;

procedure TTools.Tool1Click(Sender: TObject);
begin
  Self.MainPopupMenu.Popup();
end;

procedure TTools.Tool2MouseEnter(Sender: TObject);
begin
  Self.ToolPanel2.Color := clSkyBlue;
end;

procedure TTools.Tool2MouseLeave(Sender: TObject);
begin
  Self.ToolPanel2.Color := clCream;
end;

procedure TTools.Tool2Click(Sender: TObject);
begin
  Self.WindowState := wsMinimized;
  Self.UpdateWindowState();
end;

procedure TTools.Tool3MouseEnter(Sender: TObject);
begin
  Self.ToolPanel3.Color := clSkyBlue;
end;

procedure TTools.Tool3MouseLeave(Sender: TObject);
begin
  Self.ToolPanel3.Color := clCream;
end;

procedure TTools.Tool3Click(Sender: TObject);
begin
  if Self.WindowState = wsMaximized then
    Self.WindowState := wsNormal
  else if Self.WindowState = wsNormal then
    Self.WindowState := wsMaximized;
  Self.UpdateWindowState();
end;

procedure TTools.Tool4MouseEnter(Sender: TObject);
begin
  Self.ToolPanel4.Color := clSkyBlue;
end;

procedure TTools.Tool4MouseLeave(Sender: TObject);
begin
  Self.ToolPanel4.Color := clCream;
end;

procedure TTools.Tool4Click(Sender: TObject);
begin
  Self.Close();
end;

end.

