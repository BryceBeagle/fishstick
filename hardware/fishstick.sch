EESchema Schematic File Version 4
LIBS:connector-cache
EELAYER 30 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L connector-rescue:gcube-gamecube U1
U 1 1 5D56493D
P 4650 2050
F 0 "U1" H 4878 1888 50  0000 L CNN
F 1 "gcube" H 4878 1797 50  0000 L CNN
F 2 "gamecube:SOIC-3" H 4650 2050 50  0001 C CNN
F 3 "" H 4650 2050 50  0001 C CNN
	1    4650 2050
	1    0    0    -1  
$EndComp
$Comp
L connector-rescue:gcube-gamecube U2
U 1 1 5D59D5E9
P 3950 2050
F 0 "U2" H 4178 1888 50  0000 L CNN
F 1 "gcube" H 4178 1797 50  0000 L CNN
F 2 "gamecube:SOIC-3" H 3950 2050 50  0001 C CNN
F 3 "" H 3950 2050 50  0001 C CNN
	1    3950 2050
	1    0    0    -1  
$EndComp
$Comp
L Connector:Conn_01x03_Female J2
U 1 1 5D5AA91C
P 4700 2850
F 0 "J2" V 4546 2998 50  0000 L CNN
F 1 "Conn_01x03_Female" V 4700 2650 50  0000 R CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x03_P2.54mm_Vertical" H 4700 2850 50  0001 C CNN
F 3 "~" H 4700 2850 50  0001 C CNN
	1    4700 2850
	0    -1   1    0   
$EndComp
$Comp
L Connector:Conn_01x03_Female J1
U 1 1 5D5AC8BA
P 4000 2850
F 0 "J1" V 3846 2662 50  0000 R CNN
F 1 "Conn_01x03_Female" V 4000 3800 50  0000 R CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x03_P2.54mm_Vertical" H 4000 2850 50  0001 C CNN
F 3 "~" H 4000 2850 50  0001 C CNN
	1    4000 2850
	0    -1   1    0   
$EndComp
Wire Wire Line
	3900 2650 3900 2350
Wire Wire Line
	4000 2350 4000 2650
Wire Wire Line
	4100 2650 4100 2350
Wire Wire Line
	4600 2650 4600 2350
Wire Wire Line
	4700 2350 4700 2650
Wire Wire Line
	4800 2650 4800 2350
$EndSCHEMATC
