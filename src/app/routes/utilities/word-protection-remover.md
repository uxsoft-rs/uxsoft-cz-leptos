---
layout: ../../layouts/UtilityLayout.astro
title: MS Word Protection Remover
downloadUrl: /images/Utilities/WordProtectionRemover/wordprotectionremover.exe
imageUrl: /images/Utilities/WordProtectionRemover/wordprotectionremover.jpg
icon: KeyIcon
---

### About

Basicly the program does exactly what is in its name.
It removes the editing protection (found under Developer tab &gt; Restrict editing in Microsoft Word). <br/>
So this may save your life if you forgot your editing password in some very important document you have to make some changes in.
This of course is not the official way and may not work. It is also possible your document may get corrupted in the process, so back up your documents first!

### Motivation

I once forgot how a password from a very complex document that was probably irreplaceable.
The only way out of this problem was, you've guessed it, removing the protection.
Having a moderate to expert knowledge of Microsoft Office Open Xml document structure I quickly discovered that removing the editing protection is not impossible.
And when I knew and tested how to do it I thought out I could write a program to do that when I forget again.
The first version included many libraries (OpenXml DocumentFormat SDK from Microsoft mainly) and was not compliant with my vision of single executable application so I decided to try if I could hack this into a single file without any external libraries at all.
Mission successful.