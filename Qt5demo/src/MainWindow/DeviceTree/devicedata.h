/***************************************************************************************
 *      Copyright 2009-2011 Hikvision Digital Technology Co., Ltd.
 *      FileName        :       devicedata.h
 *      Description     :       �豸��Ϣ
 *      Modification    :       �洢�豸��ص���Ч���ݣ����ڴ�ȡ�ļ��ͽ�����ʾ��ʵʱ�޸�����
 *      Version         :       V1.0.0
 *      Time            :       2009-10,11
 *      Author          :       panyd@hikvision.com  wanggp@hikvision.com
 *****************************************************************************************/

#ifndef DEVICEDATA_H_
#define DEVICEDATA_H_

#include <QString>
#include <QList>

#include "channeldata.h"
#include "DemoPublic.h"

class ChannelData;

class DeviceData
{
public:
    friend class AddNode;
    friend class QtClientDemo;
	friend class RealPlay;
    friend class PlayBack;
	friend class SerialTransfer;
    DeviceData();
    ~DeviceData();

    void setRealPlayLabel(int value);
    int getRealPlayLabel();
    
    void setUsrID(int id);
    int getUsrID();

    void setDeviceName(QString devicename);
    QString getDeviceName();

    void setIP(QString  ip);
    QString getIP();

    void setPort(int port);
    int getPort();

    void setUsrName(QString usrname);
    QString getUsrName();

    void setPasswd(QString passwd);
    QString getPasswd();

    void setMultiCast(QString multiCast);
	QString getMultiCast();

    void setEncryptionKey(QString key);
    QString getEncryptionKey();

    NET_DVR_DEVICEINFO_V30 getDeviceInfo();

	void setDeployState(int deployed);
	int  getDeployState();

    int getZeroChanNum() const;
    void setZeroChanNum(int num);
    bool isZeroChanSupported() const;

    int modifyChannelDataChild(ChannelData *channel);

private:
    //����Ԥ����¼��0��1��
    int     m_irealplaying;
    //��½�豸�󷵻ص��û�ID���������ļ�
    int     m_iuserid;
    //��½�豸�󷵻��豸��Ϣ���������ļ�
    NET_DVR_DEVICEINFO_V30 m_deviceinfo;
    //�豸���ƣ������ļ�
    QString m_qdevicename;
    //�豸IP�������ļ�
    QString m_qip;
    //�豸�˿�,�����ļ�
    int     m_qiport;
    //�û����������ļ�
    QString m_qusername;
    //�û����룬�����ļ�
    QString m_qpassword;
	//������ >=0  ���� -1
	int m_ideployed;
	//�ಥip��ַ
	QString m_multiCast;
    //�������ܼ�钥
    QString m_qencryptionkey;
    // Number of zero channels supported by the device (from byZeroChanNum in NET_DVR_DEVICEINFO_V30)
    int m_izerochenum;
    //������ͨ���ڵ��б���������ҲҪ�����ļ���
    QList<ChannelData> m_qlistchanneldata;
};

#endif /* DEVICEDATA_H_ */

