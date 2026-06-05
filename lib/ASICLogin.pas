unit ASICLogin;

{$mode ObjFPC}{$H+}

interface

uses
  Classes, SysUtils, Forms, Controls, Graphics, Dialogs, ExtCtrls, StdCtrls,
  Buttons, ComCtrls, Menus, ASIC, ASICPrompter;

type

  { TLogin }

  TLogin = class(TForm)
    IdentifierComboBox: TComboBox;
    BottomPanel: TPanel;
    MainPopupMenu: TPopupMenu;
    TemporaryUserItem: TMenuItem;
    ProfilePanel: TPanel;
    CreatePanel: TPanel;
    Logo: TImage;
    LoginPanel: TPanel;
    HeaderPanel: TPanel;
    ProviderComboBox: TComboBox;
    Main: TPanel;
    MainPanel: TPanel;
    Selector: TPanel;
    Header: TStaticText;
    ResizeText: TStaticText;
    IdentifierText: TStaticText;
    ProviderText: TStaticText;
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
    procedure ProfilePanelClick(Sender: TObject);
    procedure CreatePanelClick(Sender: TObject);
    procedure FormClose(Sender: TObject; var CloseAction: TCloseAction);
    procedure FormCreate(Sender: TObject);
    procedure FormMouseDown(Sender: TObject; Button: TMouseButton;
      Shift: TShiftState; X, Y: Integer);
    procedure FormMouseMove(Sender: TObject; Shift: TShiftState; X, Y: Integer);
    procedure FormMouseUp(Sender: TObject; Button: TMouseButton;
      Shift: TShiftState; X, Y: Integer);
    procedure FormResize(Sender: TObject);
    procedure FormShow(Sender: TObject);
    procedure LoginPanelClick(Sender: TObject);
    procedure ResizeMouseDown(Sender: TObject; Button: TMouseButton;
      Shift: TShiftState; X, Y: Integer);
    procedure ResizeMouseMove(Sender: TObject; Shift: TShiftState; X,
      Y: Integer);
    procedure ResizeMouseUp(Sender: TObject; Button: TMouseButton;
      Shift: TShiftState; X, Y: Integer);
    procedure TemporaryUserItemClick(Sender: TObject);
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
  public

  end;

var
  Login: TLogin;

implementation

{$R *.lfm}

{ TLogin }

procedure TLogin.FormClose(Sender: TObject; var CloseAction: TCloseAction);
begin

end;

procedure TLogin.ProfilePanelClick(Sender: TObject);
begin
  Self.Close();
end;

procedure TLogin.LoginPanelClick(Sender: TObject);
begin
  Self.Hide();
  Cell.PROVIDER := Self.ProviderComboBox.Text;
  Cell.IDENTIFIER := Self.IdentifierComboBox.Text;
  if Prompter.ShowModal = mrYes then
  begin

  end;
  Self.Show();
end;

procedure TLogin.CreatePanelClick(Sender: TObject);
begin
  Self.LoginPanelClick(Sender);
end;

procedure TLogin.FormCreate(Sender: TObject);
begin
  Self.FormResize(Sender);
end;

procedure TLogin.FormMouseDown(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  if Button = mbLeft then
  begin
    FormDragging := True;
    DragX := X;
    DragY := Y;
  end;
end;

procedure TLogin.FormMouseMove(Sender: TObject; Shift: TShiftState; X,
  Y: Integer);
begin
  if FormDragging then
  begin
    Self.SetTempCursor(crSizeAll);
    Self.Left := Self.Left + X - DragX;
    Self.Top := Self.Top + Y - DragY;
  end;
end;

procedure TLogin.FormMouseUp(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  FormDragging := False;
  DragX := 0;
  DragY := 0;
end;

procedure TLogin.FormResize(Sender: TObject);
begin
  Self.IdentifierText.Left := Self.MainPanel.Width div 9;  
  Self.IdentifierComboBox.Left := Self.MainPanel.Width div 9;
  Self.ProviderText.Left := Self.MainPanel.Width div 9;
  Self.ProviderComboBox.Left := Self.MainPanel.Width div 9;
  Self.ProfilePanel.Left := (Self.MainPanel.Width div 7);
  Self.CreatePanel.Left := (Self.MainPanel.Width div 7) * 3;
  Self.LoginPanel.Left := (Self.MainPanel.Width div 7) * 5;

  Self.IdentifierText.Top := (Self.MainPanel.Height div 13) * 1;
  Self.IdentifierComboBox.Top := (Self.MainPanel.Height div 13) * 3;
  Self.ProviderText.Top := (Self.MainPanel.Height div 13) * 5;
  Self.ProviderComboBox.Top := (Self.MainPanel.Height div 13) * 7;
  Self.ProfilePanel.Top := (Self.MainPanel.Height div 9) * 7;
  Self.CreatePanel.Top := (Self.MainPanel.Height div 9) * 7;
  Self.LoginPanel.Top := (Self.MainPanel.Height div 9) * 7;

  Self.IdentifierText.Width := Self.MainPanel.Width - (Self.MainPanel.Width div 9) * 2;    
  Self.IdentifierComboBox.Width := Self.MainPanel.Width - (Self.MainPanel.Width div 9) * 2;
  Self.ProviderText.Width := Self.MainPanel.Width - (Self.MainPanel.Width div 9) * 2;
  Self.ProviderComboBox.Width := Self.MainPanel.Width - (Self.MainPanel.Width div 9) * 2;
  Self.ProfilePanel.Width := (Self.MainPanel.Width div 3);
  Self.CreatePanel.Width := (Self.MainPanel.Width div 3);
  Self.LoginPanel.Width := (Self.MainPanel.Width div 3);

  Self.IdentifierText.Height := (Self.MainPanel.Height div 4);
  Self.IdentifierComboBox.Height := (Self.MainPanel.Height div 4);
  Self.ProviderText.Height := (Self.MainPanel.Height div 4);
  Self.ProviderComboBox.Height := (Self.MainPanel.Height div 4);
  Self.ProfilePanel.Height := (Self.Height div 5);
  Self.CreatePanel.Height := (Self.Height div 5);
  Self.LoginPanel.Height := (Self.Height div 5);

  Self.BottomPanel.Height := (Self.Height div 5);

  if (Self.Height > 384) then
  begin
    Self.IdentifierText.Font.Size := Self.Height div 24;
    Self.IdentifierComboBox.Font.Size := Self.Height div 24;
    Self.ProviderText.Font.Size := Self.Height div 24;
    Self.ProviderComboBox.Font.Size := Self.Height div 24;
  end else
  begin
    Self.IdentifierText.Font.Size := 16;    
    Self.IdentifierComboBox.Font.Size := 12;
    Self.ProviderText.Font.Size := 16;      
    Self.ProviderComboBox.Font.Size := 12;
  end;
end;

procedure TLogin.FormShow(Sender: TObject);
begin

end;

procedure TLogin.ResizeMouseDown(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  if Button = mbLeft then
  begin
    FormResizing := True;
    ResizeX := X;
    ResizeY := Y;
  end;
end;

procedure TLogin.ResizeMouseMove(Sender: TObject; Shift: TShiftState; X,
  Y: Integer);
begin
  if FormResizing then
  begin
    Self.SetTempCursor(crSizeAll);
    if Self.Width + X - ResizeX >= 384 then
       Self.Width := Self.Width + X - ResizeX;
    if Self.Height + Y - ResizeY >= 216 then
       Self.Height := Self.Height + Y - ResizeY;
  end;
end;

procedure TLogin.ResizeMouseUp(Sender: TObject; Button: TMouseButton;
  Shift: TShiftState; X, Y: Integer);
begin
  FormResizing := False;
  ResizeX := 0;
  ResizeY := 0;
end;

procedure TLogin.TemporaryUserItemClick(Sender: TObject);
begin
  Self.ProviderComboBox.Text := 'canor.infinitum';
  Self.IdentifierComboBox.Text := 'temporalis.novus';
  Self.LoginPanelClick(Sender);
end;

procedure TLogin.Tool1MouseEnter(Sender: TObject);
begin
  Self.ToolPanel1.Color := clSkyBlue;    
  Self.ToolText1.Font.Color := clWhite;
end;

procedure TLogin.Tool1MouseLeave(Sender: TObject);
begin
  Self.ToolPanel1.Color := clCream;  
  Self.ToolText1.Font.Color := RGBToColor(34, 34, 34);
end;

procedure TLogin.Tool1Click(Sender: TObject);
begin
  Self.MainPopupMenu.Popup();
end;

procedure TLogin.Tool2MouseEnter(Sender: TObject);
begin
  Self.ToolPanel2.Color := clSkyBlue;     
  Self.ToolText2.Font.Color := clWhite;
end;

procedure TLogin.Tool2MouseLeave(Sender: TObject);
begin
  Self.ToolPanel2.Color := clCream;    
  Self.ToolText2.Font.Color := RGBToColor(34, 34, 34);
end;

procedure TLogin.Tool2Click(Sender: TObject);
begin
  Self.WindowState := wsMinimized;
  Self.UpdateWindowState();
end;

procedure TLogin.Tool3MouseEnter(Sender: TObject);
begin
  Self.ToolPanel3.Color := clSkyBlue;    
  Self.ToolText3.Font.Color := clWhite;
end;

procedure TLogin.Tool3MouseLeave(Sender: TObject);
begin
  Self.ToolPanel3.Color := clCream;     
  Self.ToolText3.Font.Color := RGBToColor(34, 34, 34);
end;

procedure TLogin.Tool3Click(Sender: TObject);
begin
  if Self.WindowState = wsMaximized then
    Self.WindowState := wsNormal
  else if Self.WindowState = wsNormal then
    Self.WindowState := wsMaximized;
  Self.UpdateWindowState();
end;

procedure TLogin.Tool4MouseEnter(Sender: TObject);
begin
  Self.ToolPanel4.Color := clSkyBlue;
  Self.ToolText4.Font.Color := clWhite;
end;

procedure TLogin.Tool4MouseLeave(Sender: TObject);
begin
  Self.ToolPanel4.Color := clCream;  
  Self.ToolText4.Font.Color := RGBToColor(34, 34, 34);
end;

procedure TLogin.Tool4Click(Sender: TObject);
begin
  Self.Close();
end;

end.

