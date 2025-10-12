#WindowWidth = 400
#StartHeight = 30
#EndHeight = 200
#AnimationDurationMs = 200
#LSDBloxURL = "https://lsdblox.cc"
Define.b AnimFinished = #False

Define.i Event, Quit = 0
Define.i StartTime
Define.f ElapsedTime
Define.f Progress, EasedProgress
Define.i CurrentHeight
Define.i DesktopWidth
CurrentHeight = #StartHeight

ExamineDesktops()
DesktopWidth = DesktopWidth(0)
DesktopHeight = DesktopHeight(0)

If OpenWindow(0, DesktopWidth/2 - #WindowWidth/2, DesktopHeight/2, #WindowWidth, #StartHeight, "LSDBlox launcher", #PB_Window_BorderLess | #PB_Window_SystemMenu)
  StartTime = ElapsedMilliseconds()
  ButtonGadget(1, #WindowWidth - 36, 10, 24, 24, "X")
  TextGadget(2, 10, 10, #WindowWidth/2, 24, "Hi! Welcome to the LSDBlox launcher. Press any of the buttons below to continue.")
  Repeat
    Event = WaitWindowEvent()
    If Event = #PB_Event_Gadget
      Select EventGadget()
        Case 1:
          Event = #PB_Event_CloseWindow
        Case 5:
          CompilerIf #PB_Compiler_OS = 1
            RunProgram(#LSDBloxURL)
          CompilerElseIf #PB_Compiler_OS = 2
            RunProgram("xdg-open", #LSDBloxURL, "")
          CompilerEndIf
      EndSelect
    EndIf
    
    While WindowEvent()
    Wend
    
    If AnimFinished = #False
      ElapsedTime = ElapsedMilliseconds() - StartTime
      Progress = ElapsedTime / #AnimationDurationMs
      
      If Progress < 1.0 ; WOW VERY OCOL ANYMATIONNY!
        EasedProgress = (1 - Cos(Progress * #PI)) / 2
        CurrentHeight = #StartHeight + ((#EndHeight - #StartHeight) * EasedProgress)
        ResizeWindow(0, #PB_Ignore, DesktopHeight/2 - CurrentHeight/2, #WindowWidth, CurrentHeight)
        TextGadget(3, 10, CurrentHeight-88, #WindowWidth/2, 24, "these buttons still do nothing")
        ButtonGadget(4, 10, CurrentHeight-38, #WindowWidth/2, 24, "Enable browser launching")
        ButtonGadget(5, 10, CurrentHeight-68, #WindowWidth/2, 24, "Visit the LSDBLOX site")
      Else
        AnimFinished = #True
      EndIf
    EndIf
    Delay(1)
  Until Event = #PB_Event_CloseWindow
EndIf
End
; IDE Options = PureBasic 6.21 (Linux - x64)
; CursorPosition = 51
; FirstLine = 19
; Folding = -
; Optimizer
; EnableThread
; EnableXP
; EnableWayland
; DPIAware
; Executable = ../Desktop/launch.x86_64
; CPU = 5
; Compiler = PureBasic 6.21 (Linux - x64)
; EnablePurifier